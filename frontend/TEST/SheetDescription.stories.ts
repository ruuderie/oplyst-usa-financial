import type { Meta, StoryObj } from '@storybook/vue3';

import SheetDescription from '../components/ui/sheet/SheetDescription.vue';

const meta = {
  title: 'SheetDescription',
  component: SheetDescription,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof SheetDescription>;

export default meta;
type Story = StoryObj<typeof SheetDescription>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};