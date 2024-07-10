import type { Meta, StoryObj } from '@storybook/vue3';

import PopoverTrigger from '../components/ui/popover/PopoverTrigger.vue';

const meta = {
  title: 'PopoverTrigger',
  component: PopoverTrigger,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof PopoverTrigger>;

export default meta;
type Story = StoryObj<typeof PopoverTrigger>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};