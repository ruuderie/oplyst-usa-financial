import type { Meta, StoryObj } from '@storybook/vue3';

import Separator from '../components/ui/separator/Separator.vue';

const meta = {
  title: 'Separator',
  component: Separator,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Separator>;

export default meta;
type Story = StoryObj<typeof Separator>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};