import type { Meta, StoryObj } from '@storybook/vue3';

import Textarea from '../components/ui/textarea/Textarea.vue';

const meta = {
  title: 'Textarea',
  component: Textarea,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof Textarea>;

export default meta;
type Story = StoryObj<typeof Textarea>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};